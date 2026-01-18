# Nucleus Studio

Nucleus Studio is a powerful, built-in database management GUI for your Nucleus applications. It allows you to explore, query, and modify your SQLite database directly from your browser, offering a modern alternative to CLI tools or external applications.

## Getting Started

To launch Nucleus Studio, run the following command in your project root:

```bash
nucleus studio
```

By default, it will:
1.  Look for a `nucleus.db` file or use the `DATABASE_URL` environment variable.
2.  Start a local web server at `http://localhost:3000` (or the next available port).
3.  Open your default browser.

## Features

### 🏠 Dashboard
The **Home Dashboard** gives you an instant overview of your database health:
*   **Database Stats**: Total table count, connection status.
*   **Quick Actions**: One-click access to the SQL Console or Data Browser.
*   **Tables Overview**: A quick list of your tables with row counts.

### 📊 Data Grid & CRUD
Manage your data with a full-featured spreadsheet-like interface:
*   **Create**: Add new rows via a schema-aware modal form.
*   **Read**: Browse data with **Pagination** and infinite-like scrolling.
*   **Update**: Double-click or use the **Edit (✎)** button to modify rows.
*   **Delete**: Remove unwanted data with the **Delete (🗑)** button.
*   **Export**: Download your current view as a **CSV** file for external analysis.
*   **Sort & Filter**: Click column headers to sort, or use the **Filter Bar** to search specific columns (Supports `=`, `contains`, `>`, `<`).

### 🛠 Schema Viewer
Inspect your database structure without writing queries. The **Schema** tab shows:
*   Column Names
*   Data Types
*   Nullability
*   Primary Keys

### ⌨️ SQL Console
For advanced users, the **SQL Console** provides a professional editing environment:
*   **Autocomplete**: Intelligent code completion for SQL keywords.
*   **Syntax Highlighting**: Color-coded editor for better readability.
*   **Raw Power**: Run complex `SELECT` queries, `ALTER TABLE`, or custom updates.

### 📱 Responsive Design
Nucleus Studio is fully responsive:
*   **Mobile Friendly**: Manage your database from your phone with a collapsible sidebar.
*   **Adaptive Layout**: Tables and grids adjust to your screen size.

## Configuration

Nucleus Studio automatically detects your database. You can override the target database using the environment variable:

```bash
DATABASE_URL=sqlite:./my-custom.db nucleus studio
```

> [!IMPORTANT]
> **Security Note**: Nucleus Studio is designed for **local development**. It allows unrestricted read/write access to your database. Do not expose the Studio port to the public internet in production environments.
