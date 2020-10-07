# bash ./sampling.sh <command>

loop=10
max=0
min=24*60*60
time_list=()

for ((i=0; i < ${loop}; i++)); do
  cpu_time=`(time (eval "${@}")) 2>&1 | grep user;`
  minute=$(expr `echo ${cpu_time} | sed -E "s/^.*(user.)(.+)(m)(.+)(s).*$/\2/"` )
  second=$(expr `echo ${cpu_time} | sed -E "s/^.*(user.)(.+)(m)(.+)(s).*$/\4/"` )
  minsec=`awk "BEGIN { print ${minute}*60+${second} }"`
  summary=`awk "BEGIN { print ${summary}+${minsec} }"`

  # max
  number=$(echo ${minsec} | awk "\$1>${max}{print}")
  if [ "${number}" != "" ]; then max=${number}; fi;

  # min
  number=$(echo ${minsec} | awk "\$1<${min}{print}")
  if [ "${number}" != "" ]; then min=${number}; fi;

  time_list+=( "${minsec}" )
  echo "${i}: $cpu_time"
done

# median
sorted_time_list=($(echo ${time_list[@]} | tr ' ' '\n' | sort))
median=${sorted_time_list[${loop}/2]} # 配列の要素数が奇数の場合は切り捨てで参照される

# average
average=`awk "BEGIN { print ${summary}/${loop} }"`
echo "user cpu time max:      ${max}s"
echo "user cpu time min:      ${min}s"
echo "user cpu time median:   ${median}s"
echo "user cpu time average:  ${average}s"
