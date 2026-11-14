#!/usr/bin/env bash
set -e

mkdir -p "$HOME/.agents/skills" "$HOME/.agents/rules"

workshop start dev 2>/dev/null || true
workshop refresh dev
workshop stop dev
workshop connect dev/pi:llm-endpoint dev/system:llm-endpoint
workshop remount dev/pi:skills "$HOME/.agents/skills"
workshop remount dev/pi:rules "$HOME/.agents/rules"
workshop start dev
