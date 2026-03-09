# 🧾 Agenda Descentralizada en Solana

![Solana](https://img.shields.io/badge/Solana-Blockchain-green)
![Rust](https://img.shields.io/badge/Rust-Language-orange)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blue)

## 📌 Descripción

Este proyecto implementa una **agenda descentralizada** desarrollada sobre la blockchain de Solana utilizando Rust y el framework Anchor.

El programa permite a los usuarios crear y gestionar tareas personales que se almacenan directamente en la blockchain. Cada tarea se guarda en una cuenta única utilizando **Program Derived Addresses (PDA)**, lo que garantiza seguridad, unicidad y control por parte del usuario.

Este proyecto fue desarrollado como parte de una certificación enfocada en el desarrollo de programas en Solana.

---

# 🚀 Funcionalidades

El programa implementa las operaciones **CRUD** para la gestión de tareas:

| Operación | Descripción |
|--------|-------------|
| Crear tarea | Permite crear una nueva tarea en la agenda |
| Editar tarea | Modificar título, descripción o fecha |
| Actualizar estado | Cambiar el estado de la tarea |
| Eliminar tarea | Eliminar una tarea de la blockchain |

---

# 📊 Estructura de una tarea

Cada tarea contiene la siguiente información:

- Título
- Descripción
- Fecha
- Estado (pendiente / completado)
- Autor

Ejemplo:

Título: Estudiar Solana  
Descripción: Practicar contratos inteligentes con Rust  
Fecha: 10 Marzo  
Estado: Pendiente  

---

# 🧠 Conceptos aplicados

Durante el desarrollo del proyecto se aplicaron los siguientes conceptos del ecosistema de Solana:

- Smart Contracts en Solana
- Program Derived Address (PDA)
- Gestión de cuentas en blockchain
- CRUD en programas de Solana
- Arquitectura de programas con Anchor

---

# 🏗 Arquitectura del proyecto

<img width="1024" height="1536" alt="solana" src="https://github.com/user-attachments/assets/9a762d26-9159-4f00-8f58-4641c3c65b5c" />


Cada tarea se almacena en una cuenta de la blockchain cuya dirección se genera mediante:

usuario + título de la tarea

Esto permite que cada usuario pueda tener múltiples tareas sin conflictos de direcciones.

---


# ⚙️ Tecnologías utilizadas

- Solana
- Rust
- Anchor Framework
- Solana Playground
- GitHub

---

# 🧪 Pruebas

El proyecto incluye pruebas básicas para verificar el funcionamiento del programa utilizando el entorno de desarrollo de Anchor.

Las pruebas validan:

- Creación de tareas
- Actualización de estado
- Edición de datos
- Eliminación de tareas

---

# 🎯 Objetivo del proyecto

El objetivo de este proyecto es demostrar el uso de **programas en Solana desarrollados en Rust**, aplicando operaciones CRUD y utilizando **Program Derived Addresses (PDA)** para el almacenamiento seguro de datos en la blockchain.

---

# 👩‍💻 Autor

**Sandy Judith Hernandez Carlos**

---

# 🔗 Repositorio

El código completo del proyecto se encuentra disponible en este repositorio público de GitHub.
