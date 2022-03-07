import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class PeopleService {

  private readonly _people = new BehaviorSubject<string[]>([]);

  constructor() {
    setTimeout(() => {
      this.setPeople(["John Smith", "Hans Schmied", "Jean Forgeron"]);
    }, 2000);
  }

  private setPeople(people: string[]) {
    this._people.next(people);
  }

  private currentPeople() {
    return this._people.getValue();
  }

  get people() {
    return this._people.asObservable()
  }

  addPerson(person: string) {
    this.setPeople([...this.currentPeople(), person])
  }
}
