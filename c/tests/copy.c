#include "ism/algorithm.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <glib.h>


/**
 * These tests are AI generated. They are not guaranteed to be perfect and just an experiment.
 */



int run_copyStudents_test()
{
   // Create an array of student structs
    GArray *students = g_array_new(FALSE, FALSE, sizeof(student));
    student s1 = {.mimiPoints = 10, .name = "Alice", .pSeminar = NULL, .wSeminar = NULL, .pVotes = 0, .wVotes = 0};
    student s2 = {.mimiPoints = 20, .name = "Bob", .pSeminar = NULL, .wSeminar = NULL, .pVotes = 0, .wVotes = 0};
    g_array_append_val(students, s1);
    g_array_append_val(students, s2);

    // Copy the array
    GArray *copy = copyStudents(students);

    // Check that the copy is a deep copy
    g_assert(students != copy);
    g_assert_cmpuint(students->len, ==, copy->len);
    for (guint i = 0; i < students->len; i++)
    {
        student *s = &g_array_index(students, student, i);
        student *c = &g_array_index(copy, student, i);
        g_assert(s != c);
        g_assert_cmpstr(s->name, ==, c->name);
        g_assert_cmpint(s->pVotes, ==, c->pVotes);
        g_assert_cmpint(s->wVotes, ==, c->wVotes);
    }

    // Check that the mimiPoints field is set to zero in the copy
    for (guint i = 0; i < copy->len; i++)
    {
        student *c = &g_array_index(copy, student, i);
        g_assert_cmpint(c->mimiPoints, ==, 0);
    }

    // Add more test cases
    student s3 = {.mimiPoints = 30, .name = "Charlie", .pSeminar = NULL, .wSeminar = NULL, .pVotes = 0, .wVotes = 0};
    g_array_append_val(students, s3);
    copy = copyStudents(students);
    g_assert(students != copy);
    g_assert_cmpuint(students->len, ==, copy->len);
    for (guint i = 0; i < students->len; i++)
    {
        student *s = &g_array_index(students, student, i);
        student *c = &g_array_index(copy, student, i);
        g_assert(s != c);
        g_assert_cmpstr(s->name, ==, c->name);
        g_assert_cmpint(s->pVotes, ==, c->pVotes);
        g_assert_cmpint(s->wVotes, ==, c->wVotes);
    }

    // Check that the mimiPoints field is set to zero in the copy
    for (guint i = 0; i < copy->len; i++)
    {
        student *c = &g_array_index(copy, student, i);
        g_assert_cmpint(c->mimiPoints, ==, 0);
    }

    g_array_remove_index(students, 1);
    copy = copyStudents(students);
    g_assert(students != copy);
    g_assert_cmpuint(students->len, ==, copy->len);
    for (guint i = 0; i < students->len; i++)
    {
        student *s = &g_array_index(students, student, i);
        student *c = &g_array_index(copy, student, i);
        g_assert(s != c);
        g_assert_cmpstr(s->name, ==, c->name);
        g_assert_cmpint(s->pVotes, ==, c->pVotes);
        g_assert_cmpint(s->wVotes, ==, c->wVotes);
    }

    // Check that the mimiPoints field is set to zero in the copy
    for (guint i = 0; i < copy->len; i++)
    {
        student *c = &g_array_index(copy, student, i);
        g_assert_cmpint(c->mimiPoints, ==, 0);
    }

    // Test copying an empty array
    GArray *empty = g_array_new(FALSE, FALSE, sizeof(student));
    copy = copyStudents(empty);
    g_assert(empty != copy);
    g_assert_cmpuint(empty->len, ==, copy->len);

    // Test copying an array with one element
    GArray *one = g_array_new(FALSE, FALSE, sizeof(student));
    student s4 = {.mimiPoints = 40, .name = "Dave", .pSeminar = NULL, .wSeminar = NULL, .pVotes = 0, .wVotes = 0};
    g_array_append_val(one, s4);
    copy = copyStudents(one);
    g_assert(one != copy);
    g_assert_cmpuint(one->len, ==, copy->len);
    student *s = &g_array_index(one, student, 0);
    student *c = &g_array_index(copy, student, 0);
    g_assert(s != c);
    g_assert_cmpstr(s->name, ==, c->name);
    g_assert_cmpint(s->pVotes, ==, c->pVotes);
    g_assert_cmpint(s->wVotes, ==, c->wVotes);

    // Check that the mimiPoints field is set to zero in the copy
    for (guint i = 0; i < copy->len; i++)
    {
        student *c = &g_array_index(copy, student, i);
        g_assert_cmpint(c->mimiPoints, ==, 0);
    }

    // Clean up
    g_array_free(students, TRUE);
    g_array_free(copy, TRUE);
    g_array_free(empty, TRUE);
    g_array_free(one, TRUE);
    return 0;
}