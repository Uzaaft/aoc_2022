from dataclasses import dataclass


@dataclass
class ElvCaloriesList:
    calorieslist: list


    def total_calories(self):
        return sum(self.calorieslist)




def read_file(filename: str):
    # Read calories list from file.
    # Empty lines seperates the lists.
    with open(filename, 'r') as f:
        calorieslist = f.read().splitlines()
    return calorieslist

def get_calories_list(calorieslist):
    # Return list of calories.
    # If empty line, return empty list.
    elves_calories_list = []
    individual = ElvCaloriesList([])
    for calories in calorieslist:
        if calories.strip() == '':
            elves_calories_list.append(individual)
            individual = ElvCaloriesList([])
        else:
            individual.calorieslist.append(int(calories))
    return elves_calories_list

if __name__ == '__main__':
    calorieslist = read_file('input.txt')
    calorieslist = get_calories_list(calorieslist)

    # Sort 
    calorieslist.sort(key=lambda x: x.total_calories(), reverse=True)
    # Find elf with most calories.
    print(f"The elf with the most calories has {calorieslist[0].total_calories()} calories.")

    # Keep only the top three
    top_three = calorieslist[:3]
    # Sum of calories
    sum_of_top_three = sum(elf.total_calories() for elf in top_three)
    print(f"The sum of the top three elves is {sum_of_top_three}.")
