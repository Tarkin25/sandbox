import { AbstractControl, FormArray, FormControl, FormGroup } from "@angular/forms";

function formGroup<T extends {[key: string]: any;}>(initialValues: T): FormGroup {
  const entries = Object.entries(initialValues)
  .map(([name, initialValue]) => {
    return [name, formControl(initialValue)]
  })

  const controls = Object.fromEntries(entries);

  return new FormGroup(controls)
}

export default function formControl(initialValue: Array<any>): FormArray;
export default function formControl(initialValue: {[key: string]: any;}): FormGroup;
export default function formControl(initialValue: string | boolean | number): FormControl;

export default function formControl(initialValue: any): AbstractControl {
  if (Array.isArray(initialValue)) {
    return new FormArray(initialValue.map(formControl))
  } else if (typeof initialValue === "object") {
    return formGroup(initialValue)
  } else {
    return new FormControl(initialValue)
  }
}
