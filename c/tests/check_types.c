#include <hyperon/hyperon.h>
#include <stdio.h>

#include "test.h"
#include "util.h"
#include "int_gnd.h"

void setup(void) {
}

void teardown(void) {
}

START_TEST (test_check_type)
{
    space_t space = space_new_grounding_space();
    space_add(&space, expr(atom_sym(":"), atom_sym("do"), atom_sym("Verb"), atom_ref_null()));
    atom_t verb = atom_sym("Verb");

    atom_t nonsense = atom_sym("nonsense");
    atom_t undefined = ATOM_TYPE_UNDEFINED();
    ck_assert(check_type(&space, &nonsense, &undefined));
    ck_assert(check_type(&space, &nonsense, &verb));
    atom_free(nonsense);
    atom_free(undefined);

    atom_free(verb);
    space_free(space);
}
END_TEST

START_TEST (test_validate_atom)
{
    space_t space = space_new_grounding_space();
    space_add(&space, expr(atom_sym(":"), atom_sym("a"), atom_sym("A"), atom_ref_null()));
    space_add(&space, expr(atom_sym(":"), atom_sym("b"), atom_sym("B"), atom_ref_null()));
    space_add(&space, expr(atom_sym(":"), atom_sym("foo"), expr(atom_sym("->"), atom_sym("A"), atom_sym("B"), atom_ref_null()), atom_ref_null()));

    atom_t foo = expr(atom_sym("foo"), atom_sym("a"), atom_ref_null());
    ck_assert(validate_atom(&space, &foo));
    atom_free(foo);

    space_free(space);
}
END_TEST

typedef struct _atoms_t {
    atom_t** items;
    size_t size;
} atoms_t;

struct atom_types_t {
    atom_type_t* types;
    size_t len;
};

void check_types(const atom_type_t actual, void* context) {
    size_t i = 0;
    struct atom_types_t* exp_atoms = context;

    for (i = 0; i < exp_atoms->len; ++i) {
        atom_type_t* expected = &exp_atoms->types[i];

        char* expected_str = stratomtype(expected);
        printf("expected: %s\n", expected_str);
        free(expected_str);

        /*if (atom_type_eq(expected, &actual)) {*/
            /*//exp_atoms->types[i] = atom_type_null();*/
            /*return;*/
        /*}*/
        /*++i;*/
    }

    /*char* actual_str = stratomtype(&actual);*/
    /*ck_assert_msg(0, "atom type '%s' is not expected", actual_str);*/
    /*free(actual_str);*/
}

START_TEST (test_get_atom_types)
{
    space_t space = space_new_grounding_space();
    space_add(&space, expr(atom_sym(":"), atom_sym("a"), expr(atom_sym("->"), atom_sym("C"), atom_sym("D"), atom_ref_null()), atom_ref_null()));
    space_add(&space, expr(atom_sym(":"), atom_sym("b"), atom_sym("B"), atom_ref_null()));
    space_add(&space, expr(atom_sym(":"), atom_sym("c"), atom_sym("C"), atom_ref_null()));

    atom_t a = atom_sym("a");
    atom_t a_type = expr(atom_sym("->"), atom_sym("C"), atom_sym("D"), atom_ref_null());
    atom_t call_a_c = expr(atom_sym("a"), atom_sym("c"), atom_ref_null());
    atom_t call_a_b = expr(atom_sym("a"), atom_sym("b"), atom_ref_null());

    atom_type_t call_a_c_types_arr[] = { atom_type_value(atom_sym("D")) };
    struct atom_types_t call_a_c_types = { call_a_c_types_arr, 1 };
    get_atom_types(&space, &call_a_c, &check_types, &call_a_c_types);
    /*atom_type_t call_a_b_types_arr[] = { };*/
    /*struct atom_types_t call_a_b_types = { call_a_b_types_arr, 0 };*/
    /*get_atom_types(&space, &call_a_b, &check_types, &call_a_b_types);*/
    /*atom_type_t a_types_arr[] = { atom_type_value(a_type) };*/
    /*struct atom_types_t a_types = { a_types_arr, 1 };*/
    /*get_atom_types(&space, &a, &check_types, &a_types);*/

    atom_free(call_a_b);
    atom_free(call_a_c);
    atom_free(a_type);
    atom_free(a);

    space_free(space);
}
END_TEST

void init_test(TCase* test_case) {
    tcase_set_timeout(test_case, 300); //300s = 5min.  To test for memory leaks
    tcase_add_checked_fixture(test_case, setup, teardown);
    tcase_add_test(test_case, test_check_type);
    tcase_add_test(test_case, test_validate_atom);
    tcase_add_test(test_case, test_get_atom_types);
}

TEST_MAIN(init_test);

