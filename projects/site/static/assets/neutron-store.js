/**
 * Neutron.store - Client-Side Storage Abstraction
 * 
 * A unified, cross-browser storage API for Nucleus applications.
 * Provides localStorage wrapper, reactive signals with persistence,
 * and IndexedDB abstraction for structured data.
 * 
 * @module Neutron.store
 * @version 1.0.0
 */

(function(global) {
    'use strict';

    // ═══════════════════════════════════════════════════════════════════════════
    // FEATURE DETECTION
    // ═══════════════════════════════════════════════════════════════════════════

    const hasLocalStorage = (function() {
        try {
            const test = '__neutron_test__';
            localStorage.setItem(test, test);
            localStorage.removeItem(test);
            return true;
        } catch (e) {
            return false;
        }
    })();

    const hasIndexedDB = typeof indexedDB !== 'undefined';

    // ═══════════════════════════════════════════════════════════════════════════
    // MEMORY FALLBACK (for environments without localStorage)
    // ═══════════════════════════════════════════════════════════════════════════

    const memoryStorage = {};

    // ═══════════════════════════════════════════════════════════════════════════
    // NEUTRON STORE - SIMPLE KEY-VALUE STORAGE
    // ═══════════════════════════════════════════════════════════════════════════

    const store = {
        /**
         * Set a value in storage
         * @param {string} key - Storage key
         * @param {*} value - Any JSON-serializable value
         * @returns {boolean} Success status
         */
        set: function(key, value) {
            try {
                const serialized = JSON.stringify(value);
                if (hasLocalStorage) {
                    localStorage.setItem('neutron:' + key, serialized);
                } else {
                    memoryStorage[key] = serialized;
                }
                store._notify(key, value);
                return true;
            } catch (e) {
                console.warn('[Neutron.store] Failed to set:', key, e);
                return false;
            }
        },

        /**
         * Get a value from storage
         * @param {string} key - Storage key
         * @param {*} defaultValue - Default if key doesn't exist
         * @returns {*} The stored value or default
         */
        get: function(key, defaultValue) {
            try {
                const raw = hasLocalStorage 
                    ? localStorage.getItem('neutron:' + key)
                    : memoryStorage[key];
                if (raw === null || raw === undefined) {
                    return defaultValue;
                }
                return JSON.parse(raw);
            } catch (e) {
                console.warn('[Neutron.store] Failed to get:', key, e);
                return defaultValue;
            }
        },

        /**
         * Remove a value from storage
         * @param {string} key - Storage key
         * @returns {boolean} Success status
         */
        remove: function(key) {
            try {
                if (hasLocalStorage) {
                    localStorage.removeItem('neutron:' + key);
                } else {
                    delete memoryStorage[key];
                }
                store._notify(key, undefined);
                return true;
            } catch (e) {
                console.warn('[Neutron.store] Failed to remove:', key, e);
                return false;
            }
        },

        /**
         * Clear all Neutron storage
         * @returns {boolean} Success status
         */
        clear: function() {
            try {
                if (hasLocalStorage) {
                    const keys = [];
                    for (let i = 0; i < localStorage.length; i++) {
                        const key = localStorage.key(i);
                        if (key && key.startsWith('neutron:')) {
                            keys.push(key);
                        }
                    }
                    keys.forEach(function(k) { localStorage.removeItem(k); });
                } else {
                    Object.keys(memoryStorage).forEach(function(k) { delete memoryStorage[k]; });
                }
                return true;
            } catch (e) {
                console.warn('[Neutron.store] Failed to clear:', e);
                return false;
            }
        },

        /**
         * Get all keys in Neutron storage
         * @returns {string[]} Array of keys
         */
        keys: function() {
            const result = [];
            try {
                if (hasLocalStorage) {
                    for (let i = 0; i < localStorage.length; i++) {
                        const key = localStorage.key(i);
                        if (key && key.startsWith('neutron:')) {
                            result.push(key.substring(8));
                        }
                    }
                } else {
                    Object.keys(memoryStorage).forEach(function(k) { result.push(k); });
                }
            } catch (e) {
                console.warn('[Neutron.store] Failed to get keys:', e);
            }
            return result;
        },

        /**
         * Get storage quota info
         * @returns {Promise<{used: number, quota: number, available: number}>}
         */
        quota: async function() {
            if (navigator.storage && navigator.storage.estimate) {
                const estimate = await navigator.storage.estimate();
                return {
                    used: estimate.usage || 0,
                    quota: estimate.quota || 0,
                    available: (estimate.quota || 0) - (estimate.usage || 0)
                };
            }
            // Fallback for older browsers
            return { used: 0, quota: 5 * 1024 * 1024, available: 5 * 1024 * 1024 };
        },

        // Internal: Subscribers for reactive updates
        _subscribers: {},

        _notify: function(key, value) {
            if (store._subscribers[key]) {
                store._subscribers[key].forEach(function(cb) { cb(value); });
            }
        },

        /**
         * Subscribe to changes on a key
         * @param {string} key - Storage key to watch
         * @param {function} callback - Called with new value on change
         * @returns {function} Unsubscribe function
         */
        subscribe: function(key, callback) {
            if (!store._subscribers[key]) {
                store._subscribers[key] = [];
            }
            store._subscribers[key].push(callback);
            return function() {
                store._subscribers[key] = store._subscribers[key].filter(function(cb) {
                    return cb !== callback;
                });
            };
        }
    };

    // ═══════════════════════════════════════════════════════════════════════════
    // NEUTRON SIGNALS - REACTIVE PERSISTENT STATE
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Create a reactive signal that persists to localStorage
     * @param {string} key - Storage key
     * @param {*} initialValue - Initial value if not in storage
     * @returns {object} Signal object with get/set/subscribe methods
     */
    function createSignal(key, initialValue) {
        let value = store.get(key, initialValue);
        const subscribers = [];

        return {
            /**
             * Get current value
             * @returns {*} Current signal value
             */
            get: function() {
                return value;
            },

            /**
             * Set new value (persists to storage)
             * @param {*} newValue - New value to set
             */
            set: function(newValue) {
                value = newValue;
                store.set(key, newValue);
                subscribers.forEach(function(cb) { cb(newValue); });
            },

            /**
             * Update value with a function
             * @param {function} fn - Function that receives current value and returns new value
             */
            update: function(fn) {
                this.set(fn(value));
            },

            /**
             * Subscribe to value changes
             * @param {function} callback - Called with new value on change
             * @returns {function} Unsubscribe function
             */
            subscribe: function(callback) {
                subscribers.push(callback);
                callback(value); // Initial call
                return function() {
                    const idx = subscribers.indexOf(callback);
                    if (idx > -1) subscribers.splice(idx, 1);
                };
            },

            /**
             * Get the storage key
             * @returns {string} Storage key
             */
            key: function() {
                return key;
            }
        };
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // NEUTRON DB - INDEXEDDB ABSTRACTION
    // ═══════════════════════════════════════════════════════════════════════════

    const DB_NAME = 'neutron_db';
    const DB_VERSION = 1;
    let dbInstance = null;

    /**
     * Open or create the IndexedDB database
     * @returns {Promise<IDBDatabase>}
     */
    function openDB() {
        if (dbInstance) return Promise.resolve(dbInstance);

        return new Promise(function(resolve, reject) {
            if (!hasIndexedDB) {
                reject(new Error('IndexedDB not supported'));
                return;
            }

            const request = indexedDB.open(DB_NAME, DB_VERSION);

            request.onerror = function() {
                reject(new Error('Failed to open IndexedDB'));
            };

            request.onsuccess = function(event) {
                dbInstance = event.target.result;
                resolve(dbInstance);
            };

            request.onupgradeneeded = function(event) {
                const db = event.target.result;
                // Create a generic object store for any data
                if (!db.objectStoreNames.contains('_default')) {
                    db.createObjectStore('_default', { keyPath: '_id' });
                }
            };
        });
    }

    /**
     * Ensure an object store exists
     * @param {string} storeName - Store name
     * @returns {Promise<void>}
     */
    async function ensureStore(storeName) {
        const db = await openDB();
        if (!db.objectStoreNames.contains(storeName)) {
            // Need to upgrade database version to add new store
            db.close();
            dbInstance = null;
            
            return new Promise(function(resolve, reject) {
                const request = indexedDB.open(DB_NAME, db.version + 1);
                
                request.onerror = function() {
                    reject(new Error('Failed to create object store'));
                };
                
                request.onsuccess = function(event) {
                    dbInstance = event.target.result;
                    resolve();
                };
                
                request.onupgradeneeded = function(event) {
                    const upgradedDb = event.target.result;
                    if (!upgradedDb.objectStoreNames.contains(storeName)) {
                        upgradedDb.createObjectStore(storeName, { keyPath: 'id' });
                    }
                };
            });
        }
    }

    const db = {
        /**
         * Store a value in IndexedDB
         * @param {string} storeName - Object store name
         * @param {object} value - Value to store (must have 'id' property)
         * @returns {Promise<void>}
         */
        put: async function(storeName, value) {
            await ensureStore(storeName);
            const database = await openDB();
            
            return new Promise(function(resolve, reject) {
                const tx = database.transaction(storeName, 'readwrite');
                const store = tx.objectStore(storeName);
                const request = store.put(value);
                
                request.onsuccess = function() { resolve(); };
                request.onerror = function() { reject(new Error('Failed to put value')); };
            });
        },

        /**
         * Get a value from IndexedDB
         * @param {string} storeName - Object store name
         * @param {*} id - Key to retrieve
         * @returns {Promise<*>} The stored value or undefined
         */
        get: async function(storeName, id) {
            try {
                await ensureStore(storeName);
                const database = await openDB();
                
                return new Promise(function(resolve, reject) {
                    const tx = database.transaction(storeName, 'readonly');
                    const store = tx.objectStore(storeName);
                    const request = store.get(id);
                    
                    request.onsuccess = function() { resolve(request.result); };
                    request.onerror = function() { reject(new Error('Failed to get value')); };
                });
            } catch (e) {
                return undefined;
            }
        },

        /**
         * Get all values from an object store
         * @param {string} storeName - Object store name
         * @returns {Promise<Array>} All values in the store
         */
        getAll: async function(storeName) {
            try {
                await ensureStore(storeName);
                const database = await openDB();
                
                return new Promise(function(resolve, reject) {
                    const tx = database.transaction(storeName, 'readonly');
                    const store = tx.objectStore(storeName);
                    const request = store.getAll();
                    
                    request.onsuccess = function() { resolve(request.result || []); };
                    request.onerror = function() { reject(new Error('Failed to get all values')); };
                });
            } catch (e) {
                return [];
            }
        },

        /**
         * Delete a value from IndexedDB
         * @param {string} storeName - Object store name
         * @param {*} id - Key to delete
         * @returns {Promise<void>}
         */
        delete: async function(storeName, id) {
            await ensureStore(storeName);
            const database = await openDB();
            
            return new Promise(function(resolve, reject) {
                const tx = database.transaction(storeName, 'readwrite');
                const store = tx.objectStore(storeName);
                const request = store.delete(id);
                
                request.onsuccess = function() { resolve(); };
                request.onerror = function() { reject(new Error('Failed to delete value')); };
            });
        },

        /**
         * Clear all data from an object store
         * @param {string} storeName - Object store name
         * @returns {Promise<void>}
         */
        clear: async function(storeName) {
            await ensureStore(storeName);
            const database = await openDB();
            
            return new Promise(function(resolve, reject) {
                const tx = database.transaction(storeName, 'readwrite');
                const store = tx.objectStore(storeName);
                const request = store.clear();
                
                request.onsuccess = function() { resolve(); };
                request.onerror = function() { reject(new Error('Failed to clear store')); };
            });
        },

        /**
         * Count items in an object store
         * @param {string} storeName - Object store name
         * @returns {Promise<number>} Number of items
         */
        count: async function(storeName) {
            try {
                await ensureStore(storeName);
                const database = await openDB();
                
                return new Promise(function(resolve, reject) {
                    const tx = database.transaction(storeName, 'readonly');
                    const store = tx.objectStore(storeName);
                    const request = store.count();
                    
                    request.onsuccess = function() { resolve(request.result); };
                    request.onerror = function() { reject(new Error('Failed to count')); };
                });
            } catch (e) {
                return 0;
            }
        }
    };

    // ═══════════════════════════════════════════════════════════════════════════
    // CROSS-TAB SYNCHRONIZATION
    // ═══════════════════════════════════════════════════════════════════════════

    if (typeof window !== 'undefined' && hasLocalStorage) {
        window.addEventListener('storage', function(event) {
            if (event.key && event.key.startsWith('neutron:')) {
                const key = event.key.substring(8);
                try {
                    const newValue = event.newValue ? JSON.parse(event.newValue) : undefined;
                    store._notify(key, newValue);
                } catch (e) {
                    // Ignore parse errors
                }
            }
        });
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXPORT
    // ═══════════════════════════════════════════════════════════════════════════

    const Neutron = global.Neutron || {};
    
    Neutron.store = store;
    Neutron.store.signal = createSignal;
    Neutron.db = db;
    
    // Feature flags
    Neutron.store.hasLocalStorage = hasLocalStorage;
    Neutron.store.hasIndexedDB = hasIndexedDB;

    global.Neutron = Neutron;

    // AMD/CommonJS support
    if (typeof module !== 'undefined' && module.exports) {
        module.exports = Neutron;
    } else if (typeof define === 'function' && define.amd) {
        define(function() { return Neutron; });
    }

})(typeof window !== 'undefined' ? window : this);
