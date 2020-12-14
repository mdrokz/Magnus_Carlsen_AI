for i in {1..6}
do
    #wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=$i&pid=52948" | pup 'a attr{href}' | perl -ne 'print if /^(?!.*com).*(gid=.*)/' >> "chessgame_url.txt" &
    echo "{\"link\": \"$i\"}"
done

