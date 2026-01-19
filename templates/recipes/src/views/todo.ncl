<n:view title="Todo Recipes" protected="true">
    <!-- Server Side Model Loading -->
    <n:model todos="crate::services::todo::list_todos().await" />
    
    <!-- Nucleus: Server Action Handler -->
    <n:action>
        use crate::services::todo::add_todo;

        let text = params.get("text").unwrap_or(&"".to_string()).clone();
        if !text.is_empty() {
            add_todo(text).await;
        }
    </n:action>

    <div class="container">
        <h1>✅ Todo List</h1>
        
        <!-- Add Form (POST) -->
        <form method="POST" style="display: flex; gap: 1rem; margin-bottom: 2rem;">
            <input type="text" name="text" placeholder="What needs doing?" required style="flex: 1; padding: 0.5rem;" />
            <button type="submit" style="padding: 0.5rem 1rem; background: #4facfe; border: none; color: white;">Add</button>
        </form>
        
        <ul style="list-style: none; padding: 0;">
            <n:for item="todo" in="todos">
                 <li style="background: #222; padding: 1rem; margin-bottom: 0.5rem; border-radius: 4px;">
                    <span class="id">#<n:text value="todo.id" /></span>
                    <strong class="text"><n:text value="todo.text" /></strong>
                 </li>
            </n:for>
        </ul>
        

        
        <a href="/" style="color: #4facfe;">&larr; Back to Recipes</a>
    </div>
    
    <style>
        .container { max-width: 600px; margin: 2rem auto; font-family: system-ui; background: #111; color: #fff; padding: 2rem; }
    </style>
</n:view>
