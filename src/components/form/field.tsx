import {
  FieldPath,
  FieldPathValue,
  FieldProps,
  FieldValues,
  MaybeValue,
  PartialKey,
  ResponseData,
} from "@modular-forms/solid";
import { JSX } from "solid-js";
import {
  TextField,
  TextFieldInput,
  TextFieldInputProps,
} from "../ui/text-field";
import { Label } from "../ui/label";

type ModularFormFieldProps<
  TFieldValues extends FieldValues,
  TResponseData extends ResponseData = undefined,
> = {
  Field: <TFieldName extends FieldPath<TFieldValues>>(
    props: FieldPathValue<TFieldValues, TFieldName> extends MaybeValue<string>
      ? PartialKey<
          Omit<FieldProps<TFieldValues, TResponseData, TFieldName>, "of">,
          "type"
        >
      : Omit<FieldProps<TFieldValues, TResponseData, TFieldName>, "of">,
  ) => JSX.Element;
  name: FieldPath<TFieldValues>;
  type?: TextFieldInputProps["type"];
  autocomplete?: HTMLInputElement["autocomplete"];
  ariaAutoComplete?: HTMLInputElement["ariaAutoComplete"];
};

export function ModularFormField<TFieldValues extends FieldValues>({
  Field,
  name,
  type,
  autocomplete,
  ariaAutoComplete,
}: ModularFormFieldProps<TFieldValues>) {
  const isPossiblySensitive =
    name.toLowerCase().includes("pass") ||
    name.toLowerCase().includes("key") ||
    name.toLowerCase().includes("secret");
  return (
    /* @ts-ignore - it works but types arent 100% */
    <Field name={name}>
      {(field, props) => (
        <TextField>
          <TextFieldInput
            id={field.name}
            placeholder={field.name}
            aria-invalid={!!field.error}
            aria-errormessage={`${field.name}-error`}
            data-error={field.error ? true : undefined}
            class="data-[error]:border-red-500 data-[error]:outline-red-500 focus:outline-none focus:ring-0 focus:border-0"
            type={type === undefined && isPossiblySensitive ? "password" : type}
            autocomplete={
              autocomplete == undefined && !isPossiblySensitive
                ? "on"
                : autocomplete
            }
            aria-autocomplete={ariaAutoComplete}
            // value={field.value}
            {...props}
          />
          {field.error && (
            <Label id={`${field.name}-error`} class="text-sm text-red-400">
              {field.error}
            </Label>
          )}
        </TextField>
      )}
    </Field>
  );
}
