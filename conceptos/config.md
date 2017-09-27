# Configuration Management

Se considera una buena practica separar el código aplicativo y configuración.

Las soluciones implementadas en Kubernetes para ese proposito se llaman ConfigMap y Secrets APIs.
Son ciudadanos de primera clase en Kubernetes ya que son "Kubernetes Objects" (y por lo tanto se describen en manifiestos Yaml o Json).

## ConfigMap API

- ConfigMap no gestiona los datos encriptados (lo hace Secrets API).
- Set of Key/Value
- Byte arrays are serialized as base64 encoded

### Las valores se usan en los pods de tres formas

- Environment variables
- Command-line arguments for a container
- Config files in a volume

## Ejemplo

```yaml
apiVersion: v1
 kind: ConfigMap
 metadata:
   Name: example-configmap
 data:
   # property-like keys
   game-properties-file-name: game.properties
   ui-properties-file-name: ui.properties
   # file-like keys
   game.properties: |
     enemies=aliens
     lives=3
     enemies.cheat=true
     enemies.cheat.level=noGoodRotten
    secret.code.passphrase=UUDDLRLRBABAS
    secret.code.allowed=true
    secret.code.lives=30
  ui.properties: |
    color.good=purple
    color.bad=yellow
    allow.textmode=true
    how.nice.to.look=fairlyNice
```

## Secrets API

Similar a ConfigMap API pero encriptado.

## Pros

- Integrado naturalmente en Kubernetes (via manifiestos)
- Centralizado y distribuido (via Etcd)

## Cons

- No se destacó ninguno

## Alternativas

- Spring Cloud Config pero en Java solamente