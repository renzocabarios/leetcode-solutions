## design-parking-system

## REFERENCE

- https://leetcode.com/problems/design-parking-system

## SOLUTION

``` Typescript
class ParkingSystem {
    private count: [number, number, number];
    constructor(big: number, medium: number, small: number) {
        this.count = [big, medium, small]
    }

    addCar(carType: number): boolean {
        if(this.count[carType-1] <= 0){
            return false;
        }
        this.count[carType-1]--;
        return true;
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * var obj = new ParkingSystem(big, medium, small)
 * var param_1 = obj.addCar(carType)
 */
```

## TIMESTAMP

- 6/03/2023 00:08:41
- 6/04/2023 00:06:04
