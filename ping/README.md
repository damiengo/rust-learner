# Ping

Une ré-écriture de la commande `ping` en Rust.

## Fonctionnement de ping

### ICMP

La commande ping se base sur le protocole `ICMP` pour envoyer un message à une machine sur le réseau, 
qui répondra avec une réponse. Le protocole `ICMP` est situé au même niveau que le protocole `IP` (couche réseau), 
il sert de contrôle pour les erreur de transmission.

Un packet `ICMP` se base sur la trame IP, à laquelle les informations suivantes sont ajoutées:

|      Bit 0-7     |      Bit 8-15    |     Bit 16-23    |     Bit 24-31    |
|------------------|------------------|------------------|------------------|
|     Trame IP     |     Trame IP     |     Trame IP     |     Trame IP     |
| Type de message  |        Code      |     Checksum     |     Checksum     |