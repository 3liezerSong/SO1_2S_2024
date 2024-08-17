#!/bin/bash

# Cantidad de contenedores a crear
NUM_CONTAINERS=10

# Comprobación: Verificar si Docker está instalado
if ! command -v docker &> /dev/null
then
    echo "Docker no está instalado. Instalando Docker..."
    sudo apt-get install -y docker.io
fi

# Comprobación: Verificar si el servicio Docker está en ejecución
if ! systemctl is-active --quiet docker
then
    echo "Docker no está en ejecución. Iniciando Docker..."
    sudo systemctl start docker
fi

# Genera nombres aleatorios
generate_random_name() {
  echo "container-$(openssl rand -hex 3)"
}

# Crear los contenedores
for i in $(seq 1 $NUM_CONTAINERS)
do
  CONTAINER_NAME=$(generate_random_name)
  
  # Comprobación: Verificar si el nombre ya existe
  if [ "$(docker ps -aq -f name=${CONTAINER_NAME})" ]; then
    echo "Error: El contenedor con nombre $CONTAINER_NAME ya existe. Saliendo..."
    exit 1
  fi

  docker run -d --name $CONTAINER_NAME alpine sleep 3600

  # Comprobación: Verificar si el contenedor se creó con éxito
  if [ "$(docker ps -q -f name=${CONTAINER_NAME})" ]; then
    echo "Contenedor $CONTAINER_NAME creado con éxito."
  else
    echo "Error al crear el contenedor $CONTAINER_NAME."
  fi
done
