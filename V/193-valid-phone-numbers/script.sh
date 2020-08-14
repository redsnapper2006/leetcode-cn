while IFS= read -r line
do
  t=`echo "$line" | tr -d '\r'`
  if [[ $t =~ ^[0-9]{3}\-[0-9]{3}\-[0-9]{4}$ ]]
  then
    echo $t
  elif [[ $t =~ ^\([0-9]{3}\)[\ ]{1}[0-9]{3}\-[0-9]{4}$ ]]
  then
    echo $t
  fi
done < "file.txt"
