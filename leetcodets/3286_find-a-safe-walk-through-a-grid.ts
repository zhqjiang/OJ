//https://leetcode.com/problems/find-a-safe-walk-through-a-grid/description/

function findSafeWalk(grid: number[][], health: number): boolean {
    const m = grid[0].length
    const n = grid.length


    // store maximum health
    const visited = Array.from({
        length: n,
    },
        () => Array(m).fill(-3))


    // queue for bfs
    const q: [row: number, col: number, health: number][] = [[0, 0, health - grid[0][0]]]

    // neighbor directions array
    const dirs = [[-1, 0], [1, 0], [0, 1], [0, -1]]


    // index of current node in the queue
    let index = 0;

    while (index < q.length) {
        const [x, y, currentH] = q[index]
        index++

        // check if already reach bottom right node
        if (x === n - 1 && y === m - 1) return true


        for (const dir of dirs) {
            const newX = x + dir[0]
            const newY = y + dir[1]

            if (newX >= 0 && newX < n && newY >= 0 && newY < m) {
                const newHealth = currentH - grid[newX][newY]
                if (newHealth > 0 && newHealth > visited[newX][newY]) {
                    visited[newX][newY] = newHealth
                    q.push([newX, newY, newHealth])
                }
            }
        }

    }
    return false
}