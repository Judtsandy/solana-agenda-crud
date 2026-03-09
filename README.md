# Agenda Descentralizada en Solana

## Descripción

Este proyecto implementa una agenda descentralizada utilizando la blockchain de Solana y el lenguaje Rust con el framework Anchor.

La aplicación permite a los usuarios gestionar tareas personales almacenadas directamente en la blockchain. Cada tarea se guarda en una cuenta única utilizando Program Derived Addresses (PDA), lo que garantiza seguridad y unicidad en los datos.

Este proyecto fue desarrollado como parte de una certificación enfocada en el desarrollo de programas en Solana.

---

## Funcionalidades

El programa implementa operaciones CRUD para gestionar tareas:

- Crear una tarea
- Editar una tarea
- Actualizar el estado de una tarea
- Eliminar una tarea

Cada tarea contiene la siguiente información:

- Título
- Descripción
- Fecha
- Estado (pendiente o completado)
- Autor

---

## Tecnologías utilizadas

- Solana
- Rust
- Anchor Framework
- Solana Playground

---

## Conceptos aplicados

Durante el desarrollo del proyecto se utilizaron los siguientes conceptos:

- Programas en Solana
- Program Derived Address (PDA)
- CRUD (Create, Read, Update, Delete)
- Cuentas en la blockchain
- Smart Contracts

---


---

## Cómo funciona

Cada tarea se almacena en una cuenta dentro de la blockchain de Solana.

La dirección de cada tarea se genera mediante un Program Derived Address (PDA) utilizando:

usuario + título de la tarea

Esto permite que un usuario pueda crear múltiples tareas sin conflictos de direcciones.

---

## Ejemplo de una tarea

Título: Estudiar Solana  
Descripción: Practicar contratos inteligentes con Rust  
Fecha: 10 Marzo  
Estado: Pendiente

---

## Autor

Sandy Judith Hernandez Carlos

---

## Repositorio

El código completo del proyecto se encuentra disponible en este repositorio público de GitHub.
