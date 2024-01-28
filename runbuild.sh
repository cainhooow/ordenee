#! /bin/zsh
# echo "Building current target";
# cargo tauri build
# echo "Building x86_64-pc-windows-gnu";
# cargo tauri build --target x86_64-pc-windows-gnu;

#!/bin/bash

config_file="generation.config.json"
tauri_config_file="src-tauri/tauri.conf.json"

print -P "%F{green}=== | STARTING BUILD | ===%f\n"

if [ -e "$config_file" ]; then
    current_version=$(jq -r .version "$config_file")

    # Obtendo a data atual em formato YYYYMMDDHHMMSS
    current_date=$(date "+%Y%m%d%H%M%S")

    # Obtendo a string aleatória
    generatedversion=$(tr -dc A-Za-z0-9-_ </dev/urandom | head -c 5)

    # Convertendo a versão atual para partes separadas
    IFS='-' read -r version_base hash_part <<<"$current_version"
    IFS='.' read -r major minor patch <<<"$version_base"

    if [ "$patch" -lt 10 ]; then
        patch=$((patch + 1))
    elif [ "$minor" -lt 10 ]; then
        patch=0
        minor=$((minor + 1))
    else
        # Aumenta apenas a última parte se as outras duas forem 10
        minor=0
        patch=0
        major=$((major + 1))
    fi

    new_version="$major.$minor.$patch-$current_date-$generatedversion"

    echo "Current version: $current_version"
    echo "Incremented version: $new_version"

    # Atualizando o arquivo de configuração
    jq ".version=\"$new_version\"" "$config_file" >tmp.json && mv tmp.json "$config_file"

    # Atualizando o arquivo de configuração Tauri
    jq ".package.version=\"$new_version\"" "$tauri_config_file" >tmp_tauri.json && mv tmp_tauri.json "$tauri_config_file"

    print -P "%F{green}=== | STARTING BUILD -- x86_64-pc-windows-gnu | ===%f\n"
    cargo tauri build --target x86_64-pc-windows-gnu
    print -P "%F{green}=== | STARTING BUILD -- default | ===%f\n"
    cargo tauri build
else
    print -P "%F{red}[ERR] FILE NOT EXISTS%f - %F{green}CREATING CONFIG FILE%f"

    # Obtendo a data atual em formato YYYYMMDDHHMMSS
    current_date=$(date "+%Y%m%d%H%M%S")

    # Obtendo a string aleatória
    generatedversion=$(tr -dc A-Za-z0-9-_ </dev/urandom | head -c 5)

    new_version="0.0.0-$current_date-$generatedversion"

    echo "{\"version\":\"$new_version\"}" >"$config_file"
    jq ".package.version=\"$new_version\"" "$tauri_config_file" >tmp_tauri.json && mv tmp_tauri.json "$tauri_config_file"
fi
