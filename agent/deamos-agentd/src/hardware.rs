//! Best-effort hardware detection and model recommendation for the first-boot
//! experience (A-M3). Detection is Linux-specific and degrades gracefully
//! (unknown values become 0); the recommendation itself is a pure, tested
//! function.

/// Detected hardware relevant to local inference.
#[derive(Debug, Clone, Copy)]
pub struct Specs {
    pub ram_gb: u32,
    pub vram_gb: u32,
}

/// Recommend an Ollama model id for the given hardware. Prefers GPU VRAM;
/// falls back to system RAM for CPU inference.
pub fn recommend_model(specs: &Specs) -> &'static str {
    let effective_gb = if specs.vram_gb >= 4 {
        specs.vram_gb
    } else {
        specs.ram_gb
    };
    match effective_gb {
        48.. => "llama3.3:70b",
        24.. => "qwen2.5:32b",
        12.. => "qwen2.5:14b",
        8.. => "qwen2.5:7b",
        4.. => "llama3.2:3b",
        _ => "llama3.2:1b",
    }
}

/// Detect RAM and GPU VRAM. Both are best-effort; unknown values are 0.
pub fn detect() -> Specs {
    Specs {
        ram_gb: detect_ram_gb().unwrap_or(0),
        vram_gb: detect_vram_gb().unwrap_or(0),
    }
}

fn detect_ram_gb() -> Option<u32> {
    // /proc/meminfo: "MemTotal:       16310312 kB"
    let meminfo = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = meminfo.lines().find(|l| l.starts_with("MemTotal:"))?;
    let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some((kb / 1024 / 1024) as u32)
}

fn detect_vram_gb() -> Option<u32> {
    if let Some(mib) = nvidia_vram_mib() {
        return Some((mib / 1024) as u32);
    }
    if let Some(bytes) = amd_vram_bytes() {
        return Some((bytes / 1024 / 1024 / 1024) as u32);
    }
    None
}

fn nvidia_vram_mib() -> Option<u64> {
    let out = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .next()?
        .trim()
        .parse()
        .ok()
}

fn amd_vram_bytes() -> Option<u64> {
    let rd = std::fs::read_dir("/sys/class/drm").ok()?;
    for entry in rd.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        // "card0", "card1" — skip the "card0-HDMI..." connector nodes.
        if name.starts_with("card") && !name.contains('-') {
            let path = format!("/sys/class/drm/{name}/device/mem_info_vram_total");
            if let Ok(s) = std::fs::read_to_string(&path) {
                if let Ok(bytes) = s.trim().parse::<u64>() {
                    return Some(bytes);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(ram: u32, vram: u32) -> &'static str {
        recommend_model(&Specs {
            ram_gb: ram,
            vram_gb: vram,
        })
    }

    #[test]
    fn gpu_preferred_over_ram() {
        // Big RAM but tiny GPU → GPU ignored (<4), uses RAM.
        assert_eq!(rec(32, 2), "qwen2.5:32b");
        // Real GPU drives the choice.
        assert_eq!(rec(8, 24), "qwen2.5:32b");
    }

    #[test]
    fn tiers() {
        assert_eq!(rec(64, 0), "llama3.3:70b");
        assert_eq!(rec(16, 0), "qwen2.5:14b");
        assert_eq!(rec(8, 0), "qwen2.5:7b");
        assert_eq!(rec(4, 0), "llama3.2:3b");
        assert_eq!(rec(2, 0), "llama3.2:1b");
    }

    #[test]
    fn unknown_hardware_is_safe() {
        // All-zero (detection failed) → smallest model, never panics.
        assert_eq!(rec(0, 0), "llama3.2:1b");
    }
}
