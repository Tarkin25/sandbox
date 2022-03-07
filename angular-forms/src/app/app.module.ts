import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { FirstComponent } from './components/first/first.component';
import { SecondComponent } from './components/second/second.component';
import { HomeComponent } from './components/home/home.component';
import { ReactiveFormsModule } from '@angular/forms';
import { NameEditorComponent } from './components/name-editor/name-editor.component';
import { ProfileEditorComponent } from './components/profile-editor/profile-editor.component';
import { PeopleListComponent } from './components/people-list/people-list.component';
import { AddPersonComponent } from './components/add-person/add-person.component';

@NgModule({
  declarations: [
    AppComponent,
    FirstComponent,
    SecondComponent,
    HomeComponent,
    NameEditorComponent,
    ProfileEditorComponent,
    PeopleListComponent,
    AddPersonComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    ReactiveFormsModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
