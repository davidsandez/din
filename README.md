# Din

CLI ligera escrita en Rust para realizar peticiones HTTP periódicas a un
endpoint.

Está pensada principalmente para mantener "despiertos" servidores web
que entran en estado de suspensión automática en planes gratuitos, como
ocurre en Render y otras plataformas similares.

------------------------------------------------------------------------

## 🚀 ¿Qué problema resuelve?

Muchos proveedores cloud en planes gratuitos:

-   Suspenden la aplicación tras un período de inactividad.
-   Generan "cold starts" lentos cuando llega la siguiente petición.
-   Pueden tardar varios segundos en responder la primera request.

Din envía requests HTTP periódicas para:

-   Mantener activo el proceso.
-   Evitar latencias por cold start.
-   Detectar caídas o errores del servicio.

------------------------------------------------------------------------

## ⚙️ ¿Qué hace exactamente?

-   Envía requests HTTP GET a una URL.
-   Lo hace en intervalos configurables.
-   Permite configurar timeout.
-   Muestra resultados en tiempo real.
-   Registra estadísticas.
-   Finaliza limpiamente con `Ctrl+C`.

------------------------------------------------------------------------

## 📦 Instalación

### Opción 1 --- Compilar desde el código fuente

Requisitos:

-   Rust instalado (https://rust-lang.org)

Clonar el repositorio:

``` bash
git clone https://github.com/tuusuario/din.git
cd din
```

Compilar:

``` bash
cargo build --release
```

Instalar globalmente:

``` bash
cargo install --path .
```

Esto lo deja disponible como comando global `din`.

------------------------------------------------------------------------

### Opción 2 --- Compartir el binario compilado

Din compila como binario standalone.

Después de:

``` bash
cargo build --release
```

El ejecutable estará en:

``` bash
target/release/din
```

Podés:

-   Copiarlo a `/usr/local/bin`
-   Subirlo a un servidor
-   Compartirlo directamente como archivo

Ejemplo instalación manual:

``` bash
sudo cp target/release/din /usr/local/bin/
```

------------------------------------------------------------------------

## ▶️ Uso

### Forma básica

``` bash
din https://example.com
```

Esto enviará requests cada 120 segundos (default).

------------------------------------------------------------------------

### Forma explícita

``` bash
din -u https://example.com
```

------------------------------------------------------------------------

### Con opciones

``` bash
din https://example.com -i 60 -t 5
```

### Parámetros disponibles

  Opción               Descripción                           Default
  -------------------- ------------------------------------- ---------
  `-u`, `--u`          URL a monitorear                      ---
  Posicional           URL a monitorear                      ---
  `-i`, `--interval`   Intervalo entre requests (segundos)   120
  `-t`, `--timeout`    Timeout por request (segundos)        10

------------------------------------------------------------------------

## 📊 Ejemplo de salida

``` text
🚀 Iniciando monitoreo de: https://example.com
⏱️  Intervalo: 60 segundos
⏳ Timeout: 5 segundos
============================================================
Presiona Ctrl+C para detener el monitoreo

✅ [2026-02-21 14:32:10] Petición #1: OK - Tiempo: 0.21s
```

Al detener con Ctrl+C:

``` text
============================================================
🛑 Monitoreo detenido por el usuario
============================================================
📊 Estadísticas:
   • Total de peticiones: 15
   • Exitosas: 15 (100.0%)
   • Con errores: 0 (0.0%)
============================================================
👋 ¡Hasta luego!
```

------------------------------------------------------------------------

## 🧠 Casos de uso típicos

-   Mantener activo un backend en Render.
-   Mantener despierta una API en Railway.
-   Evitar cold starts en aplicaciones serverless.
-   Supervisión básica de disponibilidad.
-   Testeo manual de estabilidad.

------------------------------------------------------------------------

## 🔒 Consideraciones

-   No reemplaza un sistema de monitoreo profesional.
-   No incluye alertas.
-   No reintenta dentro del mismo ciclo.
-   No paraleliza requests.

Es una herramienta simple, determinística y transparente.

------------------------------------------------------------------------

## 📌 Filosofía

Din es:

-   Minimalista
-   Binario único
-   Sin dependencias externas en runtime
-   Fácil de distribuir
-   Predecible

Ideal para desarrolladores que quieren algo directo, sin infraestructura
adicional.
