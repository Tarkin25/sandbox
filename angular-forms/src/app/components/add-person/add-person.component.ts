import { Component, OnInit } from '@angular/core';
import { PeopleService } from 'src/app/services/people.service';
import formControl from 'src/app/utilities/formControl';

@Component({
  selector: 'app-add-person',
  templateUrl: './add-person.component.html',
  styleUrls: ['./add-person.component.scss']
})
export class AddPersonComponent implements OnInit {

  formGroup = formControl({
    person: ""
  })

  constructor(private peopleService: PeopleService) { }

  ngOnInit(): void {
  }

  handleSubmit() {
    this.peopleService.addPerson(this.person.value);
  }

  private get person() {
    return this.formGroup.get("person")!;
  }

}
