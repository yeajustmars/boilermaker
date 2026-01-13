#!/usr/bin/env bash

boilermaker_variable_profiles_installer() {
  local LOGO=$(cat <<'LOGO_TEXT'
 ___      _ _                    _
| _ ) ___(_) |___ _ _ _ __  __ _| |_____ _ _
| _ \/ _ \ | / -_) '_| '  \/ _` | / / -_) '_|
|___/\___/_|_\___|_| |_|_|_\__,_|_\_\___|_|
LOGO_TEXT
)

  local BLUE='\033[0;34m'
  local BOLD='\033[1m'
  local GREEN='\033[0;32m'
  local ITAL='\033[3m'
  local NC='\033[0m'
  local PURPLE='\033[0;35m'
  local RED='\033[0;31m'
  local ERROR="${RED}[💥(BH::QUICK_INSTALL) ERROR]${NC}"
  local INFO="${BLUE}[📦 (BH::QUICK_INSTALL) INFO]${NC}"
  local OK="${GREEN}[📦 (BH::QUICK_INSTALL) OK]${NC}"

  echo -e "${PURPLE}${LOGO}${NC}\n"

  echo -e "${INFO} ................................................... Installing templates"

  local tpl_url="https://github.com/yeajustmars/boilermaker"
  local tpl_subdir="examples/variable-profiles"
  local tpl_basename="var-profiles"
  local tpl_langs=('node' 'python')
  local num_langs=${#tpl_langs[@]}

  local __branch_hack__="${BOIL_QUICK_INSTALL_BRANCH}"
  if [[ ! -z "$__branch_hack__" ]]; then
    echo -e "${INFO} Using branch hack: ${GREEN}${BOLD}$__branch_hack__${NC}"
    __branch_hack__="--branch $__branch_hack__"
  fi

  local num_ok=0
  for lang in "${tpl_langs[@]}"; do
    local name="${tpl_basename}-${lang}"

    echo -e "${INFO} Installing for lang: ${GREEN}${BOLD}${lang}${NC}"

    local cmd="boil install --lang $lang $tpl_url --subdir $tpl_subdir $__branch_hack__ --name $name"
    echo -e "${INFO} ${ITAL}$cmd${NC}"
    $cmd
    status=$?
    echo -e ">>>>>>>>>>>>>>>>>>>>>>>> status = $status"

    if [ $status -eq 0 ]; then
      ((num_ok++))
    else
      echo -e "${ERROR} Installation failed for lang: ${RED}${BOLD}${lang}${NC}"
      return 1
    fi
  done

  if [ $num_ok -ne $num_langs ]; then
    echo -e "${ERROR} Installation failed for some languages. Exiting."
    return 1
  fi

  echo -e "${INFO} ................................................... Creating projects from templates"

  local num_ok=0
  for lang in "${tpl_langs[@]}"; do
    local name="${tpl_basename}-${lang}"
    local dir="/tmp/${name}"
    local cmd="boil new $name --use-profile $lang -Od /tmp"

    echo -e "${INFO} ${ITAL}$cmd${NC}"
    $cmd
  done



  #node_name="${name}-node"
  #boil install --lang node "$url" -d "$subdir" -n "$node_name"
  #boil new --use-profile node "$node_name" -Od /tmp
  #cd /tmp/var-profiles-node
  #node src/main.js

  #boil install https://github.com/yeajustmars/boilermaker/ \
  #  -d examples/variable-profiles \
  #  -l python \
  #  -n var-profiles-python

  #boil new var-profiles-python -Od /tmp --use-profile python

  #cd /tmp/var-profiles-python

  #python3 src/main.py

  echo -e "${OK} Done!\n"

} && boilermaker_variable_profiles_installer
