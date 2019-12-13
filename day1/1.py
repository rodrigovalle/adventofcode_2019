def calc_fuel(mass):
    return  (mass // 3) - 2

def calc_fuel_rec(mass):
    fuel_mass_init = calc_fuel(mass)
    fuel = [fuel_mass_init]

    while fuel[-1] > 0:
        fuel.append(calc_fuel(fuel[-1]))

    return sum(fuel[:-1])

with open('mass.txt') as f:
    print(sum(calc_fuel_rec(int(line)) for line in f))
