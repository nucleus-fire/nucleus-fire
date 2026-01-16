/**
 * Mock Data for Playground Template Rendering
 * 
 * This data simulates server-side data that would normally come from
 * databases or APIs. It's used to render templates in the playground
 * without requiring a backend.
 * 
 * @module playground/mockData
 */

export let mockData = {
    // User data for user list examples
    users: [
        { name: 'Alex', email: 'alex@example.com', role: 'Admin', avatar: 'https://ui-avatars.com/api/?name=Alex&background=6366f1&color=fff' },
        { name: 'Sam', email: 'sam@example.com', role: 'Editor', avatar: 'https://ui-avatars.com/api/?name=Sam&background=10b981&color=fff' },
        { name: 'Jordan', email: 'jordan@example.com', role: 'Viewer', avatar: 'https://ui-avatars.com/api/?name=Jordan&background=f59e0b&color=fff' }
    ],

    // Blog posts for blog example
    posts: [
        { 
            title: 'Getting Started with Nucleus', 
            slug: 'getting-started',
            excerpt: 'Learn the basics of Nucleus framework in 5 minutes.',
            author: 'Alex',
            date: 'Oct 12, 2025',
            category: 'Tutorial',
            cover_image: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=800&q=80'
        },
        { 
            title: 'Why Rust?', 
            slug: 'why-rust',
            excerpt: 'Exploring the benefits of Rust for web development.',
            author: 'Sam',
            date: 'Oct 15, 2025',
            category: 'Opinion',
            cover_image: 'https://images.unsplash.com/photo-1518770660439-4636190af475?auto=format&fit=crop&w=800&q=80'
        },
        { 
            title: 'Islands Architecture Explained', 
            slug: 'islands-architecture',
            excerpt: 'How Partial Hydration works under the hood.',
            author: 'Jordan',
            date: 'Oct 20, 2025',
            category: 'Deep Dive',
            cover_image: 'https://images.unsplash.com/photo-1451187580459-43490279c0fa?auto=format&fit=crop&w=800&q=80'
        }
    ],

    // Featured post for blog hero
    featured: {
        title: 'Building Full-Stack Apps with Nucleus',
        excerpt: 'A complete guide to building modern web applications using the Nucleus framework.',
        slug: 'full-stack-nucleus'
    },

    // Dashboard statistics
    stats: {
        total_users: '12,345',
        revenue: '$89,234',
        orders: '1,234'
    },

    // Todo list for todo example
    todos: [
        { id: 1, title: 'Learn Nucleus basics', completed: true },
        { id: 2, title: 'Build a todo app', completed: false },
        { id: 3, title: 'Deploy to production', completed: false }
    ],

    // Counter state
    count: 0
};

/**
 * Reset mock data to initial state
 * Useful for testing or resetting after form submissions
 */
export function resetMockData() {
    mockData.todos = [
        { id: 1, title: 'Learn Nucleus basics', completed: true },
        { id: 2, title: 'Build a todo app', completed: false },
        { id: 3, title: 'Deploy to production', completed: false }
    ];
    mockData.count = 0;
}

// For backwards compatibility with non-module usage
if (typeof window !== 'undefined') {
    window.mockData = mockData;
    window.resetMockData = resetMockData;
}
