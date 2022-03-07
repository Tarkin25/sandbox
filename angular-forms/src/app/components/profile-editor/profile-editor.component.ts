import { Component, OnInit } from '@angular/core';
import { FormArray, FormBuilder } from '@angular/forms';
import formControl from 'src/app/utilities/formControl';

@Component({
  selector: 'app-profile-editor',
  templateUrl: './profile-editor.component.html',
  styleUrls: ['./profile-editor.component.scss']
})
export class ProfileEditorComponent implements OnInit {

  profileForm = formControl({
    firstName: "",
    lastName: "",
    address: {
      street: "",
      city: "Zürich",
      state: "",
      zip: "",
    },
    aliases: [
      "Hans",
      "Ueli",
      ""
    ]
  })

  constructor(private fb: FormBuilder) { }

  ngOnInit(): void {
  }

  onSubmit(): void {
    console.log(this.profileForm.value);
  }

  get aliases(): FormArray {
    return this.profileForm.get("aliases") as FormArray;
  }

  addAlias() {
    this.aliases.push(this.fb.control(""));
  }

}
