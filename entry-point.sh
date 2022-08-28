echo "started to setup container"
sleep 1
masacarri setup

echo "registering first user"
sleep 1
masacarri_cli adduser ${MASACARRI_USER}
printf "${MASACARRI_PASSWORD}\n${MASACARRI_PASSWORD}\n" | masacarri_cli passwd ${MASACARRI_USER}

echo "server starting"
sleep 1
masacarri
