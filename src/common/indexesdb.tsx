export class TodoDB {
    private db: IDBDatabase | null = null;
    private readonly DB_NAME = 'TodoDB';
    private readonly STORE_NAME = 'todos';

    // DB初期化
    async init() {
        return new Promise<void>((resolve, reject) => {
            const request = indexedDB.open(this.DB_NAME, 1);

            request.onerror = () => reject(request.error);

            request.onupgradeneeded = (event) => {
                const db = (event.target as IDBOpenDBRequest).result;
                db.createObjectStore(this.STORE_NAME, { keyPath: 'id', autoIncrement: true });
            };

            request.onsuccess = (event) => {
                this.db = (event.target as IDBOpenDBRequest).result;
                resolve();
            };
        });
    }

    // データの追加
    async addTodo(todo: { title: string, completed: boolean }) {
        return new Promise((resolve, reject) => {
            const transaction = this.db?.transaction([this.STORE_NAME], 'readwrite');
            const store = transaction?.objectStore(this.STORE_NAME);
            const request = store?.add(todo);

            if (request) {
                request.onsuccess = () => resolve(request.result);
                request.onerror = () => reject(request.error);
            }
        });
    }

    // データの削除
    async deleteTodo(id: number) {
        return new Promise<void>((resolve, reject) => {
            const transaction = this.db?.transaction([this.STORE_NAME], 'readwrite');
            const store = transaction?.objectStore(this.STORE_NAME);
            const request = store?.delete(id);

            if (request) {
                request.onsuccess = () => resolve();
                request.onerror = () => reject(request.error);
            }
        });
    }

    // DB全体の削除
    async deleteDatabase() {
        return new Promise<void>((resolve, reject) => {
            const request = indexedDB.deleteDatabase(this.DB_NAME);

            request.onsuccess = () => resolve();
            request.onerror = () => reject(request.error);
        });
    }
}