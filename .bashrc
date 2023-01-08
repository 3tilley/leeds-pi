#!/bin/bash
### This is a .bashrc with some niceties

# Bash niceties
alias ll='ls -Alh --color=auto --show-control-chars'
alias cd..='cd ..'

# Git aliases
alias gs='git status'
alias gd='git diff'
alias gdc='git diff --cached'
alias gdm='git diff origin/master...@'
alias gdmno='git diff --name-only origin/master...@'
alias gdni='git diff --no-index --'
alias gdu='git diff @{u}'
alias gcm='git checkout master'
alias gfom='git fetch origin master:master'
alias gau='git add -u'
alias gaa='git add -A'
alias gf='git fetch'
alias gp='git push'
alias gpu='git push -u origin HEAD'
alias gpl='git pull'
alias gplr='git pull --rebase'
alias gb='git rev-parse --abbrev-ref HEAD'

# This is a convention on where to put binaries that you want on PATH
export PATH=~/.local/bin:$PATH
  
# History operations - largely taken from https://www.thomaslaurenson.com/blog/2018-07-02/better-bash-history/
HISTTIMEFORMAT='%F %T '
HISTFILESIZE=-1
HISTSIZE=-1
HISTCONTROL=ignoredups
# Configure BASH to append (rather than overwrite the history):
shopt -s histappend
# Attempt to save all lines of a multiple-line command in the same entry
shopt -s cmdhist
# After each command, append to the history file and reread it, testing the convenience of -c -r
export PROMPT_COMMAND="${PROMPT_COMMAND:+$PROMPT_COMMAND$"\n"}history -a"
#export PROMPT_COMMAND="${PROMPT_COMMAND:+$PROMPT_COMMAND$"\n"}history -a; history -c; history -r"

check_bin() {
    if ! command -v "$1" &> /dev/null
    then
        echoerr "Warning, couldn't find: $1"
    fi
}

#alias git-https="git remote set-url origin https://github.com/$(git remote get-url origin | sed 's/https:\/\/github.com\///' | sed 's/git@github.com://')"
#alias git-ssh="  git remote set-url origin git@github.com:$(    git remote get-url origin | sed 's/https:\/\/github.com\///' | sed 's/git@github.com://')"

