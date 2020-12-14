for i in {0..142}
do
    wget --quiet -O - "https://chessgames.com/perl/chess.pl?page=$i&pid=52948" | pup 'a attr{href}' | perl -ne 'print if /^(?!.*com).*(gid=.*)/' >> "t.txt" &
done


# while read p; do
#    #echo "$p"
#     wget --quiet -O - "https://chessgames.com/$p" | pup 'a:first-of-type json{}' | jq '.[3].text' >> t1.txt &
# done <t.txt

wait