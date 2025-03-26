// ブラウザが閉じられるまで有効なデータボックスを作成
export const createLocalStorage = (key: string, value: any) => {
    localStorage
        .setItem(key, JSON.stringify(value));
}

// ブラウザが閉じられても有効なデータボックスを取得
export const getLocalStorage = (key: string) => {
    return JSON.parse(localStorage.getItem(key) || "{}");
}

// ブラウザが閉じられても有効なデータボックスを削除 
export const deleteLocalStorage = (key: string) => {
    localStorage.removeItem(key);
}

// ブラウザが閉じられても有効なデータボックスを更新
export const updateLocalStorage = (key: string, value: any) => {
    localStorage
        .setItem(key, JSON.stringify(value));
}