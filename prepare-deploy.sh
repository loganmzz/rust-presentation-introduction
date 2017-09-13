#!/usr/bin/env bash
set -uf -o pipefail

## Settings
DEPLOY_BRANCH="gh-pages"

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DEPLOY="${ROOT}-deploy"
ROOT_WORK="${ROOT}-work"

function fn_main {

    ## Prepare deploy directory
    {
        git worktree add "${ROOT_DEPLOY}" "${DEPLOY_BRANCH}" \
        && rm -rf "${ROOT_DEPLOY}"/*
    } || {
        echo "Unable to prepare deploy directory '${ROOT_DEPLOY}'" >&2
        exit 1
    }

    ## Prepare working directory
    (
        git worktree add --force "${ROOT_WORK}" "$(cd "${ROOT}"; git rev-parse HEAD)" \
        && git worktree lock --reason "Generating static site" "${ROOT_WORK}"
    ) || {
        echo "Unable to prepare working directory '${ROOT_WORK}'" >&2
        exit 1
    }

    ## Build
    (
        cd "${ROOT_WORK}/slides" \
        && npm install           \
        && npm run build
    ) || {
        echo "Unable to build slides" >&2
        exit 1
    }

    ## Move content
    (
        cd "${ROOT_WORK}/slides"                                         \
        && echo "... copy .gitignore"                                    \
        && touch "${ROOT_DEPLOY}"/.gitignore                             \
        && fn_copy_into .           assets data js lib plugin index.html \
        && fn_copy_into ./css       print reveal.css                     \
        && fn_copy_into ./css/theme league-LoganMzz.css
    ) || {
        echo "Unable to generate deploy content" >&2
        exit 1
    }

    echo "======================================="
    echo "SUCCESSFUL BUILD"
    echo "Visit ${ROOT_DEPLOY}"
    echo "======================================="
}

# Copy resources into a directory, ensuring target directory exist
#
# $2 : Relative base direcotry
# $n : File to copy
function fn_copy_into {
    if [[ $# -eq 0 ]]; then
        echo "Missing base directory" >&2
        return 1
    fi
    local basedir="$1"; shift
    local dst="${ROOT_DEPLOY}/${basedir}"


    echo "... copy ${basedir}"
    
    if [[ $# -eq 0 ]]; then
        echo "Missing files to copy into ${dst}" >&2
        return 1
    fi
    
    if [[ ! -d "${dst}" ]]; then
        mkdir -p "${dst}" \
        || {
            echo "Can't create directory ${dst}" >&2
            return 1
        }
    fi

    while [[ $# -ne 0 ]]; do
        local file="$1"; shift
        local src="${basedir}/${file}"
        cp -r "${src}" "${dst}" \
        || {
            echo "Can't copy from ${src} into ${dst}"
            return 1
        }
    done
}

function fn_clean {
    git worktree unlock "${ROOT_WORK}"
    rm -rf "${ROOT_WORK}" "${ROOT_DEPLOY}"
    git worktree prune

    echo "======================================="
    echo "SUCCESSFUL CLEAN"
    echo "======================================="
}

command="main"
if [[ $# -ge 1 ]]; then
    command="$1"
    shift
fi

case "${command}" in
    clean|main)
        "fn_${command}" $@
        ;;

    *)
        echo "Unsupported command ${command}" >&2
        exit 1
        ;;
esac
