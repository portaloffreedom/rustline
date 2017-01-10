DIR="$( dirname "$0" )"
RUSTLINE_COMMAND=$DIR/target/release/rustline

_rustline_append_precmd_function() {
        if test -z "${precmd_functions[(re)$1]}" ; then
                precmd_functions+=( $1 )
        fi
}

integer _POWERLINE_JOBNUM

_rustline_set_jobnum() {
    # If you are wondering why I am not using the same code as I use for bash
    # ($(jobs|wc -l)): consider the following test:
    #     echo abc | less
    #     <C-z>
    # . This way jobs will print
    #     [1]  + done       echo abc |
    #            suspended  less -M
    # ([ is in first column). You see: any line counting thingie will return
    # wrong number of jobs. You need to filter the lines first. Or not use
    # jobs built-in at all.
    _RUSTLINE_JOBNUM=${(%):-%j}
}

color () {
    echo "%{$bg[cyan]%}"
}

prompt_left() {
    #echo $RUSTLINE_COMMAND left $@
    #color
    prompt=$($RUSTLINE_COMMAND left $@)
    echo "$prompt"
}

_rustline_setup_prompt() {
    emulate -L zsh
    autoload -U colors && colors

    _rustline_append_precmd_function _rustline_set_jobnum

    #VIRTUAL_ENV_DISABLE_PROMPT=1

    add_args=''

    add_args+=' --last_exit_code=$?'
    add_args+=' --last_pipe_status="$pipestatus"'
    add_args+=' --shortened_path="${(%):-%~}"'
    add_args+=' --jobnum=$_RUSTLINE_JOBNUM'

    PROMPT='$(prompt_left '$add_args')'
    RPROMPT='$("$RUSTLINE_COMMAND" right '$add_args')'
}

setopt promptsubst
setopt promptpercent

_rustline_setup_prompt
