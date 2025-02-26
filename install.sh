#!/bin/bash
# Move the binary to /usr/local/bin ...
sudo cp target/release/pi_wifi_connector /usr/local/bin/pi_wifi_connector
# ..make it executable ...
sudo chmod +x /usr/local/bin/pi_wifi_connector
# ..edit crontab ...
# Define the cron job lines
CRON_JOB_REBOOT="@reboot /usr/local/bin/pi_wifi_connector /home/pi/my_wifi_networks.yaml 2&>1 1>/dev/null"
CRON_JOB_EVERY_MINUTE="* * * * * /usr/local/bin/pi_wifi_connector /home/pi/my_wifi_networks.yaml 2&>1 1>/dev/null"

# Get the current user's crontab
CURRENT_CRON=$(crontab -l 2>/dev/null)

# Check if the cron jobs already exist
if echo "$CURRENT_CRON" | grep -Fxq "$CRON_JOB_REBOOT" && echo "$CURRENT_CRON" | grep -Fxq "$CRON_JOB_EVERY_MINUTE"; then
    echo "Cron jobs already exist. No changes made."
else
    # Add the new cron jobs and update crontab
    (echo "$CURRENT_CRON"; echo "$CRON_JOB_REBOOT"; echo "$CRON_JOB_EVERY_MINUTE") | crontab -
    echo "Cron jobs added successfully!"
fi