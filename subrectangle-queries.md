## subrectangle-queries

## REFERENCE

- https://leetcode.com/problems/subrectangle-queries/

## SOLUTION

``` typescript
class SubrectangleQueries {
    private rectangle: number[][] = [];
    constructor(rectangle: number[][]) {
        this.rectangle = rectangle;
    }

    updateSubrectangle(row1: number, col1: number, row2: number, col2: number, newValue: number): void {
        for(let i = row1; i <= row2; i++){
            for(let j = col1; j <= col2; j++){
                this.rectangle[i][j] = newValue;
            }
        }
    }

    getValue(row: number, col: number): number {
        return this.rectangle[row][col];
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * var obj = new SubrectangleQueries(rectangle)
 * obj.updateSubrectangle(row1,col1,row2,col2,newValue)
 * var param_2 = obj.getValue(row,col)
 */

```


## TIMESTAMP

- 5/25/2023 00:18:19
- 5/26/2023 00:05:45
