_basher___guff() {
	local cur prev opts
	COMPREPLY=()
	cur="${COMP_WORDS[COMP_CWORD]}"
	prev="${COMP_WORDS[COMP_CWORD-1]}"
	opts=()
	if [[ ! " ${COMP_LINE} " =~ " -h " ]] && [[ ! " ${COMP_LINE} " =~ " --help " ]]; then
		opts+=("-h")
		opts+=("--help")
	fi
	if [[ ! " ${COMP_LINE} " =~ " -V " ]] && [[ ! " ${COMP_LINE} " =~ " --version " ]]; then
		opts+=("-V")
		opts+=("--version")
	fi
	if [[ ! " ${COMP_LINE} " =~ " -e " ]] && [[ ! " ${COMP_LINE} " =~ " --expanded " ]]; then
		opts+=("-e")
		opts+=("--expanded")
	fi
	if [[ ! " ${COMP_LINE} " =~ " -b " ]] && [[ ! " ${COMP_LINE} " =~ " --browsers " ]]; then
		opts+=("-b")
		opts+=("--browsers")
	fi
	if [[ ! " ${COMP_LINE} " =~ " -i " ]] && [[ ! " ${COMP_LINE} " =~ " --input " ]]; then
		opts+=("-i")
		opts+=("--input")
	fi
	if [[ ! " ${COMP_LINE} " =~ " -o " ]] && [[ ! " ${COMP_LINE} " =~ " --output " ]]; then
		opts+=("-o")
		opts+=("--output")
	fi
	opts=" ${opts[@]} "
	if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
		COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
		return 0
	fi
	case "${prev}" in
		--input|--output|-i|-o)
			if [ -z "$( declare -f _filedir )" ]; then
				COMPREPLY=( $( compgen -f "${cur}" ) )
			else
				COMPREPLY=( $( _filedir ) )
			fi
			return 0
			;;
		*)
			COMPREPLY=()
			;;
	esac
	COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
	return 0
}
complete -F _basher___guff -o bashdefault -o default guff
