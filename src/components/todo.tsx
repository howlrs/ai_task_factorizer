import { Button, Card, Col, Row, Select, Space, Spin } from "antd";
import { useState } from "react";
import TextArea from "antd/es/input/TextArea";
import { invoke } from "@tauri-apps/api/core";

interface Todo {
    title?: string;
    summary?: string;
    issues?: Issue[];
    created_at?: string;
}

interface Issue {
    title?: string;
    description?: string;
    progression?: number;
    estimated_working_hours?: number;
}

export const TodoComponent = () => {
    // ロード処理を管理
    const [loading, setLoading] = useState<boolean>(false);
    // 初回リソースである必要情報
    const [resource, setResource] = useState<string>('');

    // Todo保存
    const [todos, setTodos] = useState<Todo[]>([]);

    // 分解処理
    const onDisassemble = async () => {
        setLoading(true);
        try {
            const res = await invoke("greet", { resource });

            const newTodo = res as Todo;
            console.log(newTodo);

            setResource('');
            setTodos((prev) => [...prev, newTodo]);

        } catch (error) {
            console.error(error);
        } finally {
            setLoading(false);
        }
    };


    return (
        <Spin spinning={loading}>
            <Row gutter={[16, 16]}>
                <Col span={24}>
                    <Space direction="vertical" style={{ width: '100%' }}>
                        <TextArea value={resource} rows={12} onChange={(e) => setResource(e.target.value)} />
                        <Button type={
                            (todos) ? 'default' : 'primary'
                        } onClick={onDisassemble}>因子分解</Button>
                    </Space>
                </Col>
            </Row>

            <Row gutter={[16, 16]} wrap>
                <Col span={24}>
                    {
                        todos.map((todo, parentIndex) => {
                            return (
                                <Row gutter={[16, 16]} wrap key={parentIndex}>
                                    <Col span={24}>
                                        <h3>{todo.title}</h3>
                                        <p>{todo.summary}</p>
                                        <p>{todo.created_at}</p>
                                        <Row gutter={[16, 16]} wrap>
                                            {
                                                todo.issues && todo.issues.map((issue, index) => (
                                                    <Col span={12} key={index}>
                                                        <Card title={issue.title}
                                                            style={{ backgroundColor: issue.progression === 2 ? 'lightgreen' : issue.progression === 1 ? 'lightblue' : 'lightyellow' }}
                                                            extra={
                                                                <Button size="small" onClick={() => {
                                                                    setTodos((prev) => {
                                                                        if (prev.length < 1) {
                                                                            return prev;
                                                                        }
                                                                        prev[parentIndex].issues?.splice(index, 1);
                                                                        return [...prev];
                                                                    })
                                                                }}>削除</Button>
                                                            }
                                                        >
                                                            <div style={{ textAlign: 'left' }}>
                                                                <p>{issue.description}</p>
                                                                <p style={{ textAlign: 'right' }}>目安: {issue.estimated_working_hours}時間</p>
                                                            </div>
                                                            <Select onChange={(value) => {
                                                                setTodos((prev) => {
                                                                    if (prev.length < 1) {
                                                                        return prev;
                                                                    }
                                                                    const issues = prev[parentIndex].issues?.[index];
                                                                    if (!issues) {
                                                                        return prev;
                                                                    }

                                                                    console.log(value, typeof value);


                                                                    issues.progression = parseInt(value.toString());
                                                                    // 変更を親要素に反映
                                                                    prev[parentIndex].issues?.splice(index, 1, issues);
                                                                    return [...prev];
                                                                })
                                                            }} defaultValue={issue.progression ? issue.progression : "0"} style={{ width: '100%' }}>
                                                                <Select.Option value="0">未着手</Select.Option>
                                                                <Select.Option value="1">進行中</Select.Option>
                                                                <Select.Option value="2">完了</Select.Option>
                                                            </Select>
                                                        </Card>
                                                    </Col>
                                                ))
                                            }
                                        </Row>
                                    </Col>
                                </Row>
                            )
                        })
                    }
                </Col>
            </Row>
        </Spin>
    );
}