getUrl() {
    local x=$(wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=$1&pid=52948")

    local bytes=$(echo $x | wc -c)

    # echo $bytes

    # [ "$bytes" == "6148" ] || [ "$bytes" == "6147" ]

    if [ "$bytes" -gt "6000" ]
    then
        echo $x | pup 'a attr{href}' | perl -ne 'print if /^(?!.*com).*(gid=.*)/' >> "chessgame_url.txt"
    else
        getUrl $1 &
    fi
}

for i in {1..144}
do
    #wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=$i&pid=52948" | pup 'a attr{href}' | perl -ne 'print if /^(?!.*com).*(gid=.*)/' >> "chessgame_url.txt" &
    getUrl $i &
done



#while read p; do
#wget --quiet -O - "https://chessgames.com/$p" &
#    #echo "$p"
#     wget --quiet -O - "https://chessgames.com/$p" | pup 'a:first-of-type json{}' | jq '.[3].text' >> t1.txt &
#done <t.txt

wait