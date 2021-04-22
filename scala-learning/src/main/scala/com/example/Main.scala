package com.example

object Main extends App {

  case class Vector2(x: Int, y: Int) {
    def +(other: Vector2): Vector2 = Vector2(x + other.x, y + other.y)
  }

  val playerPosition = Vector2(0, 0)
  val playerVelocity = Vector2(5, 0)
  val newPlayerPosition = playerPosition + playerVelocity

  println(s"playerPosition = $playerPosition, playerVelocity = $playerVelocity, newPlayerPosition = $newPlayerPosition")

  val age = 18

  println(
    age match {
      case 18 => "barely old enough"
      case _ => "i dont care"
    }
  )

  case class Person(name: String, lastName: String)

  implicit class AdvancedPerson(person: Person) {
    def fullName(): String = s"${person.name} ${person.lastName}"
  }

  val person = Person("John", "Doe").copy(name = "Max")

  println(person.fullName())

}
