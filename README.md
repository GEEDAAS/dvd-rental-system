# 🎬 DVD Rental System

Sistema de renta de DVDs desarrollado como práctica académica, que integra:

- 🦀 Backend en **Rust**
- 🐘 Base de datos **PostgreSQL**
- 📦 Contenedores con **Docker**
- ☸️ Orquestación con **Kubernetes**
- 🖥️ Aplicación de escritorio con **Tauri**
- 🔄 Automatización CI/CD con **GitHub Actions**

---

## 👨‍🎓 Autor

- **Nombre:** Gerardo Jorge Guerrero Frausto
- **Nombre:** Alan Orlando Leonel Hinojosa Gonzalez
- **Nombre:** Carlos Antonio Aguilar Bueno
- **Nombre:** Maribel Garcia Mora  
- **Carrera:** Ingeniería en Sistemas Computacionales  
- **Materia:** Topiocos para el despliegue de aplicaciones  
- **Institución:** Tecnológico Nacional de México  

---

## 🧱 Arquitectura del Sistema

```text
┌──────────────────────────┐
│  Aplicación de Escritorio│
│        (Tauri)           │
└───────────┬──────────────┘
            │ HTTP (API)
┌───────────▼──────────────┐
│     Backend en Rust      │
│   (Docker / Kubernetes)  │
└───────────┬──────────────┘
            │ SQL
┌───────────▼──────────────┐
│     PostgreSQL (DB)      │
│   (Docker / Kubernetes)  │
└──────────────────────────┘

```

## ⚙️ Tecnologías Utilizadas

| Tecnología | Uso |
|----------|-----|
| Rust | Desarrollo del backend (API REST) |
| PostgreSQL | Base de datos relacional |
| Docker | Contenerización de servicios |
| Kubernetes (Minikube) | Orquestación de contenedores |
| Tauri | Aplicación de escritorio |
| GitHub Actions | Integración y despliegue continuo (CI/CD) |

---

## 🐳 Ejecución con Docker (Modo Desarrollo)

Este modo se utiliza para desarrollo y pruebas locales.

### 📍 Ubicación
Desde la **raíz del proyecto**: dvd_rental_system/

### ▶️ Comando

```bash
docker-compose up --build
```

La **API** quedará disponible en: http://localhost:8080

---

## ☸️ Despliegue en Kubernetes (Minikube)

### 1️⃣ Iniciar **Minikube**

```bash
minikube start
```

### 2️⃣ Crear **Namespace**

```bash
kubectl apply -f k8s/namespace.yaml
```

### 3️⃣ Desplegar **Backend y Servicios**

```bash
kubectl apply -f k8s/backend/
```

### 4️⃣ Verificar **Pods**

```bash
kubectl get pods -n dvd-system
```

### 5️⃣ Acceso mediante **Ingress**
El backend queda expuesto mediante un dominio local:

```bash
http://dvd-api.local/api/rentals/overdue
```

---

## 🖥️ Aplicación de Escritorio (Tauri)

La aplicación de escritorio consume la API desplegada en Kubernetes.

### 📍 Ubicación
dvd-rental-ui/

### 🔨 Construcción del instalador

```bash
npm install
npm run tauri build
```

### 📦 Resultado
El instalador se genera en:

```bash
dvd-rental-ui/src-tauri/target/release/bundle/
```

---

## 🔄 CI/CD con GitHub Actions

El proyecto cuenta con automatización completa mediante **GitHub Actions**.

### 🔁 Flujo de trabajo

1. Se crea un tag (ejemplo: v1.0.1)
2. GitHub Actions se activa automáticamente
3. Se compila la aplicación de escritorio
4. Se crea un Release en GitHub
5. El instalador se adjunta al Release

---

## 📦 Descargar Instalador

### 📍 Ubicación
Los instaladores generados automáticamente se encuentran en: 👉 GitHub → Releases

Cada release incluye el instalador listo para su ejecución en Windows.

---

## 🧪 Evidencias de Funcionamiento

- Backend ejecutándose en Kubernetes
- Servicios accesibles mediante Ingress
- Base de datos PostgreSQL funcionando
- GitHub Actions ejecutándose correctamente
- Instalador de escritorio generado y funcional

---

## 🏁 Conclusión

Este proyecto demuestra la integración completa de un sistema distribuido moderno, utilizando contenedores, orquestación, aplicaciones de escritorio y automatización CI/CD, cumpliendo con los objetivos académicos y buenas prácticas de ingeniería de software.