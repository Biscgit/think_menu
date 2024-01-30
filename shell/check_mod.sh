# check if module is loaded
result=$(lsmod | grep "^ideapad_laptop" 2>/dev/null)

if [ "$(echo "$result" | head -c1 | wc -c)" -eq 0 ]; then
  exit 1
fi

# finished check
exit 0
