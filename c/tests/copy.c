#include <CUnit/CUnit.h>
#include "../src/headers/algorithm.h"
/**
 * This test was an experiment and AI generated
 */

static void test_copyStudents(void) {
    // Create an array of student structs
    GArray *students = g_array_new(FALSE, FALSE, sizeof(student));
    student s1 = {.mimiPoints = 10, .name = "Alice", .pSeminar = NULL, .wSeminar = NULL, .pVotes = 0, .wVotes = 0};
    student s2 = {.mimiPoints = 20, .name = "Bob", .pSeminar = NULL, .wSeminar = NULL, .pVotes = 0, .wVotes = 0};
    g_array_append_val(students, s1);
    g_array_append_val(students, s2);

    // Copy the array
    GArray *copy = copyStudents(students);

    // Check that the copy is a deep copy
    CU_ASSERT_PTR_NOT_EQUAL(students, copy);
    CU_ASSERT_EQUAL(students->len, copy->len);
    for (guint i = 0; i < students->len; i++) {
        student *s = &g_array_index(students, student, i);
        student *c = &g_array_index(copy, student, i);
        CU_ASSERT_PTR_NOT_EQUAL(s, c);
        CU_ASSERT_EQUAL(s->mimiPoints, c->mimiPoints);
        CU_ASSERT_STRING_EQUAL(s->name, c->name);
        CU_ASSERT_PTR_EQUAL(&s->pSeminar, &c->pSeminar);
        CU_ASSERT_PTR_EQUAL(&s->wSeminar, &c->wSeminar);
        CU_ASSERT_EQUAL(s->pVotes, c->pVotes);
        CU_ASSERT_EQUAL(s->wVotes, c->wVotes);
    }

    // Clean up
    g_array_free(students, TRUE);
    g_array_free(copy, TRUE);
}

int run_copyStudents_test() {
    CU_initialize_registry();
    CU_pSuite suite = CU_add_suite("copyStudents", 0, 0);
    CU_add_test(suite, "test_copyStudents", test_copyStudents);
    CU_run_all_tests();
    CU_cleanup_registry();
    return 0;
}