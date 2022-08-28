masacarri_cli adduser ${MASACARRI_USER}
printf "${MASACARRI_PASSWORD}\n${MASACARRI_PASSWORD}\n" | masacarri_cli passwd ${MASACARRI_USER}
masacarri