import { Component, OnInit } from '@angular/core';
import { Observable } from 'rxjs';
import { PeopleService } from 'src/app/services/people.service';

@Component({
  selector: 'app-people-list',
  templateUrl: './people-list.component.html',
  styleUrls: ['./people-list.component.scss']
})
export class PeopleListComponent implements OnInit {

  people: Observable<string[]>;

  constructor(peopleService: PeopleService) {
    this.people = peopleService.people;
  }

  ngOnInit(): void {
  }

}
