#!/usr/bin/env bash
# Bake the local AI stack into the image: Ollama as a loopback systemd service.
# More of the stack (whisper.cpp, piper, embeddings) lands here in later steps.
set -euo pipefail

echo "Installing Ollama into the image..."
curl -fsSL https://ollama.com/download/ollama-linux-amd64.tgz -o /tmp/ollama.tgz
tar -C /usr -xzf /tmp/ollama.tgz
rm -f /tmp/ollama.tgz

# Create the ollama system user at boot (image-friendly via sysusers.d).
cat > /usr/lib/sysusers.d/ollama.conf <<'EOF'
u ollama - "Ollama" /usr/share/ollama
EOF

# Loopback-only model server — local inference, nothing exposed to the network.
cat > /usr/lib/systemd/system/ollama.service <<'EOF'
[Unit]
Description=Ollama local model server
After=network-online.target
Wants=network-online.target

[Service]
Type=exec
ExecStart=/usr/bin/ollama serve
User=ollama
Group=ollama
Restart=always
RestartSec=3
Environment=HOME=/usr/share/ollama
Environment=OLLAMA_HOST=127.0.0.1:11434

[Install]
WantedBy=multi-user.target
EOF

systemctl enable ollama.service
command -v ollama
echo "Ollama installed: $(ollama --version 2>/dev/null || echo unknown)"
