echo -e "I'm ask.sh. What you like to do?, enter a Task Id from list below: \n"
echo -e "TaskID\t Description"
echo -e "1\t Test via curl LOCAL"
echo -e "2\t Print Currencies"
echo -e "3\t Download"
echo -e "4\t / empty"
echo -e "5\t /help "

until [ "$task" = "0" ]; do
read task

if [ "$task" = "1" ]; then
    echo "...${task}"
    curl -X GET 'http://0.0.0.0:8087?s=eur&t=thb&v=2'

elif [ "$task" = "2" ]; then
    echo "...${task}"
    curl -X GET 'http://0.0.0.0:8087/currencies'
elif [ "$task" = "3" ]; then
    echo "...${task}"
    curl -X GET 'http://0.0.0.0:8087/download'
elif [ "$task" = "4" ]; then
    echo "...${task}"
    curl -X GET 'http://0.0.0.0:8087/'
elif [ "$task" = "5" ]; then
    echo "...${task}"
    curl -X GET 'http://0.0.0.0:8087/help'        
else
    echo "Goodbye! - Exit"
fi


sleep 3
./ask.sh

done 
