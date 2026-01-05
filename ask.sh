echo -e "I'm ask.sh. What you like to do?, enter a Task Id from list below: \n"
echo -e "TaskID\t Description"
echo -e "1\t Test via curl LOCAL"

until [ "$task" = "0" ]; do
read task

if [ "$task" = "1" ]; then
    echo "...${task}"
    curl -X GET 'http://0.0.0.0:8087?s=eur&t=thb&v=2'
    
else
    echo "Goodbye! - Exit"
fi


sleep 3
./ask.sh

done 
