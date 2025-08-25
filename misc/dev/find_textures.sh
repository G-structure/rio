#!/usr/bin/env bash
rg -n --glob '!target' '(TextureDescriptor|create_texture|SurfaceConfiguration)'
