getUrl() {
    local x=$(wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=$1&pid=52948")

    local bytes=$(echo $x | wc -c)

    # echo $bytes

    # [ "$bytes" == "6148" ] || [ "$bytes" == "6147" ]

    if [ "$bytes" -gt "6000" ]; then
        local links=($(echo $x | pup 'a attr{href}' | perl -ne 'print if /^(?!.*com).*(gid=.*)/'))
        local moves=($(echo $x | pup 'td[align="RIGHT"]:nth-child(4) > font text{}'))
        # local titles=($(echo $x | pup 'font[face="verdana,arial,helvetica"] > a text{}' | grep vs))
        moves=("${moves[@]:1}")

        local json_string=""

        for i in ${!moves[@]}; do
            local move=${moves[$i]}
            local link=${links[$i]}
            json_string="$json_string{\"link\": \"$link\",\"move\": \"$move\"},"
        done

        echo $json_string >>chess_game.json
    else
        getUrl $1 &
    fi
}

echo "[" >chess_game.json

for i in {1..144}; do
    #wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=$i&pid=52948" | pup 'a attr{href}' | perl -ne 'print if /^(?!.*com).*(gid=.*)/' >> "chessgame_url.txt" &
    getUrl $i &
done

wait

echo "]" >>chess_game.json
