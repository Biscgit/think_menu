# constants
file_path="/etc/sudoers.d/think_menu"
files=("conservation_mode" "fn_lock" "fan_mode" "usb_charging")

# get user with root permissions
sudo_user=$(grep -E "^wheel:|^sudo:" /etc/group | cut -d: -f1)

if [ "$(echo "$sudo_user" | head -c1 | wc -c)" -eq 0 ]; then
  echo "No user with higher privileges found"
  exit 1
fi

# truncate file or create new
if [ -e "$file_path" ]; then
  truncate -s 0 "$file_path"
else
  touch $file_path || exit 1
fi

# notes in file
{
  echo "## Custom settings for think_menu";
  echo "## created with least-privilege in mind";
  echo "";
} | tee -a $file_path > /dev/null;

# all write permissions
for file in "${files[@]}"
do
  line="%$sudo_user ALL=(ALL) NOPASSWD: /usr/bin/tee /sys/bus/platform/drivers/ideapad_acpi/VPC????\:??/$file";
  echo "$line" | tee -a $file_path > /dev/null;
done

# finished setup
exit 0