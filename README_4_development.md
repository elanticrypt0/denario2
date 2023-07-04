#   Denario base

La base de Denario sirve para crear mejores aplicaciones que además se pueden utilizar online.

Tiene dos routes accesibles 

1) /app_run_setup
2) /app_check_status

## /app_run_setup

Si el archivo .env tiene _ APP_SETUP_ENEABLE _ en true entonces permite crear carpetas y archivos necesarios para correr la aplicación

## /app_check_status

Este archivo lo que hace es chequear el estado de aplicación y si tiene todo para poder utilizarse correctamente.

# DB

Lo primero que hay que hacer es ejecutar las migraciones con

    diesel migration run

# Requerimientoss

Para usar el modo de desarrollo hay que instalar dos herramientas de rust

    cargo install diesel

    cargo install watch

Luego de instalar esas dos herramientas hay que ejecutar

    diesel setup
    diesel migration run

