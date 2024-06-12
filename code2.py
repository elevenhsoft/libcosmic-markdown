class Student:
    def __init__(self, name, age, grades):
        self.name = name
        self.age = age
        self.grades = grades

    def average_grade(self):
        return sum(self.grades) / len(self.grades) if self.grades else 0


class StudentManagementSystem:
    def __init__(self):
        self.students = []

    def add_student(self, student):
        self.students.append(student)

    def list_students(self):
        for student in self.students:
            print(f"Name: {student.name}, Age: {student.age}, Grades: {student.grades}, Average Grade: {student.average_grade()}")

    def find_student(self, name):
        for student in self.students:
            if student.name == name:
                return student
        return None

    def calculate_overall_average(self):
        if not self.students:
            return 0
        total_grades = []
        for student in self.students:
            total_grades.extend(student.grades)
        return sum(total_grades) / len(total_grades) if total_grades else 0


def main():
    system = StudentManagementSystem()

    # Adding students
    system.add_student(Student("Alice", 20, [85, 90, 92]))
    system.add_student(Student("Bob", 22, [78, 81, 85]))
    system.add_student(Student("Charlie", 19, [88, 79, 94]))

    # Listing all students
    print("Listing all students:")
    system.list_students()

    # Finding a student by name
    student_name = "Bob"
    found_student = system.find_student(student_name)
    if found_student:
        print(f"\nFound student: {found_student.name}, Age: {found_student.age}, Grades: {found_student.grades}, Average Grade: {found_student.average_grade()}")
    else:
        print(f"\nStudent with name {student_name} not found.")

    # Calculating overall average grade
    overall_average = system.calculate_overall_average()
    print(f"\nOverall average grade of all students: {overall_average}")


if __name__ == "__main__":
    main()
