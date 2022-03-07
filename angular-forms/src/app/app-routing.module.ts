import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { FirstComponent } from './components/first/first.component';
import { HomeComponent } from './components/home/home.component';
import { SecondComponent } from './components/second/second.component';

const routes: Routes = [
  { path: "", component: HomeComponent },
  { path: "first", component: FirstComponent},
  { path: "second", component: SecondComponent},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
